type ToastOption = {
  duration: number;
  description: string;
  title: string;
};

const baseOptions = {
  duration: 9000,
  isClosable: true
};

const baseErrorMessage = {
  ...baseOptions,
  title: "An error occurred.",
  status: "error"
};

const baseSuccessMessage = {
  ...baseOptions,
  title: "Success",
  status: "success"
};

export const errorFetchStats: ToastOption = {
  ...baseErrorMessage,
  description: "Error while fetching stats fot this domain."
};

export const errorCreateWebsite: ToastOption = {
  ...baseErrorMessage,
  description: "Unable to create new website for this user."
};

export const errorFetchWebsites: ToastOption = {
  ...baseErrorMessage,
  description: "Unable to find websites for this user."
};

export const errorLogin: ToastOption = {
  ...baseErrorMessage,
  description: "Unable to login user account."
};

export const errorRegister: ToastOption = {
  ...baseErrorMessage,
  description: "Unable to create user account."
};

export const errorNewWebsite: ToastOption = {
  ...baseErrorMessage,
  description: "Unable to create website."
};

export const successCreateWebsite : ToastOption = {
  ...baseSuccessMessage,
  description: "You just created a new website!"
};
