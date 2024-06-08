export const createNuxtNotFoundError = <P>(props?: P) => {
  return createError({
    statusCode: 404,
    name: 'NotFoundError',
    statusMessage: 'Not Found',
    message: 'Not Found!',
    fatal: true,
    data: props,
  });
};
