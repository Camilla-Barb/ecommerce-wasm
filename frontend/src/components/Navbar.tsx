import React from "react";

export const Navbar: React.FC = () => {
  return (
    <nav className="navbarWrapper" aria-label="Main Navigation">
      <ul className="navbarList">
        <li>
          <a href="/">Home</a>
        </li>
        <li>
          <a href="/about">Wishlist</a>
        </li>
        <li>
          <a href="/contact">Cart</a>
        </li>
      </ul>
    </nav>
  );
};
