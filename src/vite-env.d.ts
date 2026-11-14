declare module 'virtual:action-validator' {
  const validate: import('ajv').ValidateFunction<import('./types/actions').ActionDescriptor>;
  export default validate;
}
