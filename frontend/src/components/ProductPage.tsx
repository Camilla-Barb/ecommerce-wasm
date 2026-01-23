import React from "react";
import { ProductList } from "./ProductList";
import "../styles/ProductPage.css";

export const ProductPage: React.FC = () => {
  return (
    <div className="plpPageContainer">
      <img src="/images/hero.png" alt="hero image" className="heroImg"></img>
      <h1 className="sr-only">Products - Winter Collection</h1>
      <div className="productListWrapper">
        <ProductList />
      </div>
    </div>
  );
};
