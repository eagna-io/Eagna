export const getAccessToken = (): string | null => {
  return window.sessionStorage.getItem("token");
};

export const setAccessToken = (token: string) => {
  window.sessionStorage.setItem("token", token);
};

export const getAdminAccessToken = (): string | null => {
  return window.sessionStorage.getItem("admin-token");
};

export const setAdminAccessToken = (token: string) => {
  window.sessionStorage.setItem("admin-token", token);
};
