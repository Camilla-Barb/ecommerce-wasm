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
    <div>
      <ul>
        {data!.products.map((product, idx) => (
          <li key={idx} className="card">
            {product.imageUrl && (
              <img
                src={product.imageUrl || "/images/placeholder.png"}
                alt={product.name}
              />
            )}
            <h2>{product.name}</h2>
            <p>${product.price}</p>
            {product.description && <h3>{product.description}</h3>}
          </li>
        ))}
      </ul>
    </div>
  );
};
