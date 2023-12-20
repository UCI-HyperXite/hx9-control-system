import "./style.css";

export default function Navbar() {
  return (
    <header id="navbar">
      <img
        alt="hx logo"
        id="hxlogo"
        src={require("../../Data/Images/HX Logo.png")}
      />
      HyperXite 9
    </header>
  );
}
