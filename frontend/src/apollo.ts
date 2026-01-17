import { ApolloClient, InMemoryCache, HttpLink } from "@apollo/client";

const link = new HttpLink({
  uri: "http://127.0.0.1:3000/graphql",
});

export const client = new ApolloClient({
  cache: new InMemoryCache(),
  link,
});
