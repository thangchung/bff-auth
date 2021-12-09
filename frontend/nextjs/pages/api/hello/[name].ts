// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next'
import axios, { normalizeCookie } from 'lib/axios'
import { AxiosError } from 'axios'

type Data = {
  name: string
}

type Error = {
  errorCode: number,
  errorMessage: string
}

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data | Error>
) {
  const { name } = req.query
  try {
    let user = await axios.get(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/sale/${name}`, {
      headers: {
        cookie: normalizeCookie(req.cookies)
      }
    })

    res.status(200).json({ name: user.data })
  } catch (error: any) {
    const err = error as AxiosError
    if (err.response) {
      res.status(error.response.status).json({ errorCode: err.response.status, errorMessage: err.response.statusText })
    }
  }
}
