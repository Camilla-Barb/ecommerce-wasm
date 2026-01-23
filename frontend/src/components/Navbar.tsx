import React from "react";
import "../styles/Navbar.css";

export const Navbar: React.FC = () => {
  return (
    <nav className="navbarWrapper" aria-label="Main Navigation">
      <ul className="navbarList">
        <li>
          <a href="/">Home</a>
        </li>
        <li>
          <a href="/wishlist">Wishlist</a>
        </li>
        <li>
          <a href="/cart">Cart</a>
        </li>
      </ul>
    </nav>
  );
};
