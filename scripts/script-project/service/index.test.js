const micro = require('micro')
const fetch = require('node-fetch')
const helloWorld = require('.')

test('responds with hello world', () => {
  const response = await fetch('http://localhost:3000')
  const json = await response.json()
  expect(json.greeting).toEqual('hello world')
})
