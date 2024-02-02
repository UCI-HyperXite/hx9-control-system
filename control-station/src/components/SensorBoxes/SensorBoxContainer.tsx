import CCcontainer from "./Camera&Console/CCcontainer"
import SensorContainer from "./Sensors/SensorsContainer"
import "./style.css"
export default function SensorBoxContainer(){
    return(
        <div id="sensorboxcontainer">
            <SensorContainer />
            <CCcontainer />
        </div>
    )
}