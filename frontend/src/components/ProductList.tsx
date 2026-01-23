import React from "react";
import { useQuery } from "@apollo/client";
import { GET_PRODUCTS } from "../graphql/queries";
import type { Product } from "../types/product";
import "../styles/ProductList.css";

interface ProductsData {
  products: Product[];
}

export const ProductList: React.FC = () => {
  const { loading, error, data } = useQuery<ProductsData>(GET_PRODUCTS);

  if (loading) return <p>Loading...</p>;
  if (error) return <p>Error: {error.message}</p>;
  if (!data) return null;

  return (
    <ul className="grid">
      {data!.products.map((product, idx) => (
        <li key={idx} className="card">
          <img
            src={product.imageUrl || "/images/placeholder.png"}
            alt={product.name}
          />

          <h2>{product.name}</h2>
          {product.description && (
            <h3 className="productDesc">{product.description}</h3>
          )}
          <p>${product.price}</p>
        </li>
      ))}
    </ul>
  );
};
