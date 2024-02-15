import CCcontainer from "./camera&console/CCcontainer"
import SensorContainer from "./sensors/SensorsContainer"
import "./sensorboxct.css"
export default function SensorBoxContainer(){
    return(
        <div id="sensorboxcontainer">
            <SensorContainer />
            <CCcontainer />
        </div>
    )
}