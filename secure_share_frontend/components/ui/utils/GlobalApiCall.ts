/* eslint-disable react-hooks/rules-of-hooks */
import { useCurrentUserServer } from "@/hooks/use-current-user-server";
import { RedirectError } from "./ErrorUtils";

interface GlobalApiCallProps {
  url: string;
  options?: RequestInit;
}

export const GlobalApiCall = async ({
  url,
  options = {},
}: GlobalApiCallProps) => {
  try {
    const session = await useCurrentUserServer();

    const token = session?.accessToken ?? null;

    const response = await fetch(url, {
      ...options,
      credentials: "include",
      headers: {
        "Content-Type": "application/json",
        ...(token ? { Authorization: `Bearrer ${token}` } : {}),
        ...options.headers,
      },
    });

    if (response.status === 401) {
      throw new RedirectError(302, "/logout", "session expired");
    }

    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(
        `Http error! status: ${response.status}, message: ${errorText}`
      );
    }
    return await response.json();
  } catch (error) {
    console.error("fetch Error:", error);
    throw error;
  }
};
