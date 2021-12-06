import type { NextApiRequest, NextApiResponse } from 'next'
import axios, { extractCookie } from 'lib/axios'
import { AxiosError } from 'axios'

type Data = {
    isLogin: boolean
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
                cookie: extractCookie(req.cookies)
            }
        })

        res.status(200).json({ isLogin: Object.keys(user.data).length !== 0 })
    } catch (error: any) {
        const err = error as AxiosError
        if (err.response) {
            res.status(error.response.status).json({ errorCode: err.response.status, errorMessage: err.response.statusText })
        }
    }
}
