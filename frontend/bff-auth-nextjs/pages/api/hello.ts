// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next'
import axios, { extractCookie } from 'lib/axios'

type Data = {
  name: string
}

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {
  let user = await axios.get(`${process.env.NEXT_PUBLIC_BACKEND_URL}/api/John Doe`, {
    headers: {
      cookie: extractCookie(req.cookies)
    }
  })

  res.status(200).json({ name: user.data })
}
