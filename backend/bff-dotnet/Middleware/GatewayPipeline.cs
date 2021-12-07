using Gateway.Config;
using Gateway.Services;

namespace Gateway.Middleware;

public static class GatewayPipeline
{
    private static bool IsExpired(HttpContext ctx)
    {
        var expiresAt = Convert.ToInt64(ctx.Session.GetString(SessionKeys.ExpiresAt)) - 30;
        var now = new DateTimeOffset(DateTime.Now).ToUnixTimeSeconds();

        var expired = now >= expiresAt;
        return expired;
    }

    private static bool HasRefreshToken(HttpContext ctx)
    {
        var refreshToken = ctx.Session.GetString(SessionKeys.RefreshToken);
        return !string.IsNullOrEmpty(refreshToken);
    }

    private static string GetRefreshToken(HttpContext ctx)
    {
        var refreshToken = ctx.Session.GetString(SessionKeys.RefreshToken);
        return refreshToken ?? "";
    }

    private static async Task Refresh(HttpContext ctx, TokenRefreshService tokenRefreshService)
    {
        var refreshToken = GetRefreshToken(ctx);

        var resp = await tokenRefreshService.RefreshAsync(refreshToken);

        if (resp == null)
        {
            // Next call to API will fail with 401 and client can take action
            return;
        }

        var expiresAt = new DateTimeOffset(DateTime.Now).AddSeconds(Convert.ToInt32(resp.expires));

        ctx.Session.SetString(SessionKeys.AccessToken, resp.access_token);
        ctx.Session.SetString(SessionKeys.IdToken, resp.id_token);
        ctx.Session.SetString(SessionKeys.RefreshToken, resp.refresh_token);
        ctx.Session.SetString(SessionKeys.ExpiresAt, "" + expiresAt.ToUnixTimeSeconds());
    }

    public static void UseGatewayPipeline(this IReverseProxyApplicationBuilder pipeline)
    {
        var tokenRefreshService = pipeline.ApplicationServices.GetRequiredService<TokenRefreshService>();
        var config = pipeline.ApplicationServices.GetRequiredService<GatewayConfig>();

        var apiPath = config.ApiPath;

        pipeline.Use(async (ctx, next) =>
        {
            if (IsExpired(ctx) && HasRefreshToken(ctx))
            {
                await Refresh(ctx, tokenRefreshService);
            }

            var token = ctx.Session.GetString(SessionKeys.AccessToken);
            var currentUrl = ctx.Request.Path.ToString().ToLower();

            if (!string.IsNullOrEmpty(token) && currentUrl.StartsWith(apiPath))
            {
                ctx.Request.Headers.Add("Authorization", "Bearer " + token);
            }

            await next().ConfigureAwait(false);
        });
    }
}