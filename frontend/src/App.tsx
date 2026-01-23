import { ApolloProvider } from "@apollo/client/react";
import { client } from "./apollo/client";
import { ProductPage } from "./components/ProductPage";
import { Navbar } from "./components/Navbar";

function App() {
  return (
    <ApolloProvider client={client}>
      <Navbar />
      <ProductPage />
    </ApolloProvider>
  );
}

export default App;
