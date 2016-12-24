import %substitute_project_name from '.'

test('outputs hello world', () => (
  expect(%substitute_project_name()).toBe('hello world')
))
