import { ApolloProvider } from "@apollo/client/react";
import { client } from "./apollo";
import { ProductList } from "./ProductList";

function App() {
  return (
    <ApolloProvider client={client}>
      <ProductList />
    </ApolloProvider>
  );
}

export default App;
