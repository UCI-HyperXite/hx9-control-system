import "./style.css"
import HX from "../../../public/Data/Images/HX Logo.png"
export default function Navbar(){
    return(
        <header id="navbar">
            <img src={HX} style={{height:"60px"}}/>
            HyperXite</header>
    )
}