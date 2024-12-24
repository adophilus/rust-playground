import { serve } from '@hono/node-server'
import { Hono } from 'hono'

const app = new Hono()
const PORT = 5000

app.get('/', (c) => {
  return c.text('Hello Hono!')
})

serve({
  fetch: app.fetch,
  port: PORT
}, ({ port, address }) =>
  console.log(`Server is running on http://${address}:${port}`)
)
