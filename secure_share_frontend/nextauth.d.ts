import { DefaultSession } from "next-auth";

//Extend the DefaultSession["user"] type with accessToken
export type ExtendedUser = DefaultSession["user"] & {
  accessToken: string;
};

declare module "next-auth" {
  //Extend user interface with accessTken
  interface User {
    token: string;
  }

  //Extend Session interface with the extended user
  interface Session {
    user: ExtendedUser;
  }
}
