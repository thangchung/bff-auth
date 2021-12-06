namespace Gateway.Config;

public record GatewayConfig
{
    public string Url { get; init; } = "";
    public int SessionTimeoutInMin { get; init; }
    public string ApiPath { get; init; } = "";
    public string Authority { get; init; } = "";
    public string ClientId { get; init; } = "";
    public string ClientSecret { get; init; } = "";
    public string Scopes { get; init; } = "";
    public string LogoutUrl { get; init; } = "";
}