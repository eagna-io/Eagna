export const getAccessToken = (): string | null => {
  return window.sessionStorage.getItem("token");
};

export const setAccessToken = (token: string) => {
  window.sessionStorage.setItem("token", token);
};
