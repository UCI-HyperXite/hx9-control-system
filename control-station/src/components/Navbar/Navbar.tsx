import HX from "@/data/images/HX Logo.svg";

import "./Navbar.css";

export default function Navbar() {
	return (
		<header className="navbar">
			<img alt="HX logo" src={HX} style={{ height: "60px" }} />
			HyperXite
		</header>
	);
}
