import "./style.css"
import Camera from "./Camera";
import Console from "./Console";
export default function CCcontainer(){
    return(
        <div id="CCcontainer">
            <Camera />
            <Console />
        </div>
    )
}