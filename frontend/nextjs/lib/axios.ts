import Axios from 'axios'
import https from 'https'

const axios = Axios.create({
    baseURL: process.env.NEXT_PUBLIC_BACKEND_URL,
    headers: {
        'X-Requested-With': 'XMLHttpRequest',
    },
    withCredentials: true,
    httpsAgent: new https.Agent({
        rejectUnauthorized: false
    })
})

export function normalizeCookie(cookies: any) {
    return Object.keys(cookies).map(function(k){return `${k}=${cookies[k]}` }).join(",")
}

export default axios