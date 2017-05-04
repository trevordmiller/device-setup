const fetch = require('node-fetch')

test('responds with hello world', async () => {
  const response = await fetch('http://localhost:3000')
  const json = await response.json()
  expect(json.greeting).toEqual('hello world')
})
