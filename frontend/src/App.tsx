import { ApolloProvider } from "@apollo/client/react";
import { client } from "./apollo/client";
import { ProductPage } from "./components/ProductPage";

function App() {
  return (
    <ApolloProvider client={client}>
      <ProductPage />
    </ApolloProvider>
  );
}

export default App;
