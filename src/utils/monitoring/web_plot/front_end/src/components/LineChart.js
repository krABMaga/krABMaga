import React, {useState, useEffect } from 'react';
import axios from 'axios'
import { createRoot } from 'react-dom/client'
import Chart from 'chart.js/auto';
import ReactDOM from 'react-dom';


const LineChart = () => {
  let socket = new WebSocket("ws://127.0.0.1:3012");
  
  socket.onopen = function(e) {
    console.log("[open] Connection established");
  };
  
  socket.onmessage = function(event) {
    console.log(`[message] Data received from server: ${event.data}`);
    const obj = JSON.parse(event.data);
    console.log(obj);
    let insert = true;
    for(let object in chartData){
      console.log("ciao");
      if(object.id === obj.file){
        console.log("ciao");
        insert = false;
      }
    }
    if(insert){
      let dataSet = [];
        let max = 0;
        for(let j = 0; j<obj.dataset.length;j++){
          let color = randomColor();
          dataSet.push({
            label:obj.dataset[j].header,
            data:obj.dataset[j].values,
            backgroundColor: color,
            borderColor : color
          });
          if(obj.dataset[j].values.length > max)
            max = obj.dataset[j].values.length;
        }
        const data = {
          labels: Array.from(Array(max).keys()),
          datasets: dataSet,
        };
      let new_dataset = {
        id : obj.file,
        data : data,
      };
      setChartData(current => [...current,new_dataset]);
    }
  };
  
  socket.onclose = function(event) {
    if (event.wasClean) {
      alert(`[close] Connection closed cleanly, code=${event.code} reason=${event.reason}`);
    } else {
      // e.g. server process killed or network down
      // event.code is usually 1006 in this case
      console.log('[close] Connection died');
    }
  };
  
  socket.onerror = function(error) {
    alert(`[error] ${error.message}`);
  };
      const [data, setData] = useState([]);
      const url = 'http://127.0.0.1:8000/getcsvdata'
      var divs = [];
      useEffect(() => {
        getData();
      }, []);
  
        const getData = () => {
          axios.get(url)
          .then((response)=>{
              const myData = response.data;
              console.log(myData);
              setData(myData);
              for(let i=0;i<myData.length;i++){
                let element = React.createElement(
                  "canvas",
                  {key:i, id:"canvas"+i,height:"40vh", width:"80vw"},
                  );
                divs.push(element);
              }

              /*
              const root = ReactDOM.createRoot(
                document.getElementById('myDiv')
              );
              root.render(divs);
              */

              ReactDOM.render(
                divs,
                document.getElementById("myDiv")
              );
              for(let i = 0; i<myData.length;i++){
                let dataSet = [];
                let labels = [];
                let max = 0;
                for(let j = 0; j<myData[i].dataset.length;j++){
                  const label=myData[i].dataset[j].header;
                  const data = myData[i].dataset[j].values;
                  let color = "black";
                  const oggetto = {
                    label:label,
                    data:data,
                    backgroundColor: color,
                    borderColor : color
                  }
                  dataSet.push(oggetto);
                }
                const ctx = document.getElementById('canvas'+i).getContext('2d');
                let chartStatus = Chart.getChart("canvas"+i);
                if (chartStatus !== undefined) {
                    chartStatus.destroy();
                }
                for(var k = 1; k<=10;k++){
                  labels.push(k.toString());
                }
                const data = {
                    labels: labels,
                    datasets: dataSet,
                  };
                const myChart = new Chart(ctx, {
                    type: 'line',
                    data: data,
                    options: {
                      responsive:true
                    }
                });
              }
          });
        };
  
          return (
              <div>
                <div id="myDiv"></div>
              </div>
          );
        }
      
      export default LineChart;