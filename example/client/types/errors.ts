export class NotFoundError extends Error {
  statusCode = 404;
  name = 'NotFoundError';

  constructor(message = 'Not Found!') {
    super(message);
  }
}

export class InsufficientBalanceError extends Error {
  name = 'InsufficientBalance';
  constructor(message = 'Insufficient balance!') {
    super(message);
  }
}

export class NotInstalledError extends Error {
  name = 'NotConnected';
  constructor(message = 'Not installed!') {
    super(message);
  }
}

export class NotConnectedError extends Error {
  name = 'NotConnected';
  constructor(message = 'Connect to execute contracts!') {
    super(message);
  }
}

export class NoContractAddressError extends Error {
  name = 'NoContractAddress';
  constructor(message = 'No contract address provided!') {
    super(message);
  }
}
