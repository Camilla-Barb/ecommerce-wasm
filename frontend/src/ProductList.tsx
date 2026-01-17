import React from "react";
import { useQuery } from "@apollo/client";
import { GET_PRODUCTS } from "./graphql/queries";

export const ProductList: React.FC = () => {
  type Product = {
    name: string;
    price: number;
  };

  type ProductsQueryData = {
    products: Product[];
  };
  const { loading, error, data } = useQuery<ProductsQueryData>(GET_PRODUCTS);

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error.message}</p>;

  return (
    <div>
      <h1>Products</h1>
      <ul>
        {data!.products.map((product, idx) => (
          <li key={idx}>
            {product.name} - ${product.price}
          </li>
        ))}
      </ul>
    </div>
  );
};
