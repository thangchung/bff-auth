import type { NextApiRequest, NextApiResponse } from 'next'
import axios, { normalizeCookie } from 'lib/axios'
import { AxiosError } from 'axios'

type Data = {
    isLogin: boolean,
    name: string,
}

type Error = {
    errorCode: number,
    errorMessage: string
}

export default async function handler(
    req: NextApiRequest,
    res: NextApiResponse<Data | Error>
) {
    try {
        let user = await axios.get(`${process.env.NEXT_PUBLIC_BACKEND_URL}/userinfo`, {
            headers: {
                cookie: normalizeCookie(req.cookies)
            }
        })

        if (Object.keys(user.data).length !== 0) {
            res.status(200).json({
                isLogin: true,
                name: user.data.name
            })
        } else {
            res.status(200).json({
                isLogin: false,
                name: ''
            })
        }
    } catch (error: any) {
        const err = error as AxiosError
        if (err.response) {
            res.status(error.response.status).json({ errorCode: err.response.status, errorMessage: err.response.statusText })
        }
    }
}
