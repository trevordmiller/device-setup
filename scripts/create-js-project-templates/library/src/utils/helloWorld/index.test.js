import helloWorld from '.'

test('outputs hello world', () => {
  expect(helloWorld()).toBe('hello world')
})
