import "./navbar.css"
import HX from "@/data/images/HX Logo.svg"
export default function Navbar(){
    return(
        <header className="navbar">
            <img alt="HX logo" src={HX} style={{height:"60px"}}/>
            HyperXite</header>
    )
}