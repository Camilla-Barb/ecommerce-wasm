import { ApolloClient, InMemoryCache } from "@apollo/client";

export const client = new ApolloClient({
  uri: "http://127.0.0.1:3000/graphql", // must point exactly to the BE
  cache: new InMemoryCache(),
});
