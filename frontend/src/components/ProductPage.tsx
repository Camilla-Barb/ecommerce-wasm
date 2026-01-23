import React from "react";
import { ProductList } from "./ProductList";
import "../styles/ProductPage.css";

export const ProductPage: React.FC = () => {
  return (
    <div className="plpPageContainer">
      <img src="/images/hero.png" alt="hero image" className="heroImg"></img>
      <div className="plpName">
        <h1 className="plpTitle">Products - Winter Collection</h1>
        <p className="plpParagraph"> Find your look</p>
      </div>
      <div className="productListWrapper">
        <ProductList />
      </div>
    </div>
  );
};
