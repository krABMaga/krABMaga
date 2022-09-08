import {useState, useEffect } from 'react'
import axios from 'axios'
import "../styles/Graph.css";
import { Link } from 'react-router-dom';
import AOS from "aos";
import "aos/dist/aos.css";
import {FaArrowUp} from 'react-icons/fa';

import { 
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  BarElement,
  PointElement,
  LineElement,
  Title,
  Tooltip,
  Legend,
  Filler,
} from 'chart.js'
import {Line} from 'react-chartjs-2'

ChartJS.register(
  Filler,
  LineElement,
  CategoryScale,
  LinearScale,
  BarElement,
  PointElement,
  Title,
  Tooltip,
  Legend,
);

function randomInteger(max) {
  return Math.floor(Math.random()*(max + 1));
}

function randomRgbaColor() {
  let r = randomInteger(255);
  let g = randomInteger(255);
  let b = randomInteger(255);
  let a = 1;
  return "rgba("+r+","+g+","+b+","+a+")";
}

function opacityRgba(rgba) {
  return rgba.replace("1)","0.1)")
}

function getOrCreateColors(id,data,localStorageColor){
  console.log(id);
  console.log(data);
  console.log(localStorageColor);
  if(localStorageColor.has(id)){
    console.log("ci sono");
  }else{
    let colors = [];
    for(let i = 0; i < data.datasets.length; i++){
      let colore = randomRgbaColor();
      console.log(colore);
      colors.push(colore);
    }
    console.log(colors);
    localStorageColor.set(id,colors);
  }
  window.sessionStorage.setItem('colors', JSON.stringify(localStorageColor, replacer));
  return localStorageColor.get(id);
}

function removeChartColor(id,localStorageColor){
  localStorageColor.delete(id);
  window.sessionStorage.setItem('colors', JSON.stringify(localStorageColor, replacer));
}

function replacer(key, value) {
  if(value instanceof Map) {
    return {
      dataType: 'Map',
      value: Array.from(value.entries()), // or with spread: value: [...value]
    };
  } else {
    return value;
  }
}

function reviver(key, value) {
  if(typeof value === 'object' && value !== null) {
    if (value.dataType === 'Map') {
      return new Map(value.value);
    }
  }
  return value;
}

const getOrCreateTooltip = (chart) => {
  let tooltipEl = chart.canvas.parentNode.querySelector('div');

  if (!tooltipEl) {
    tooltipEl = document.createElement('div');
    tooltipEl.style.background = 'rgba(0, 0, 0, 0.7)';
    tooltipEl.style.borderRadius = '3px';
    tooltipEl.style.color = 'white';
    tooltipEl.style.opacity = 1;
    tooltipEl.style.pointerEvents = 'none';
    tooltipEl.style.position = 'absolute';
    tooltipEl.style.transform = 'translate(-50%, 0)';
    tooltipEl.style.transition = 'all .1s ease';

    const table = document.createElement('table');
    table.style.margin = '0px';

    tooltipEl.appendChild(table);
    chart.canvas.parentNode.appendChild(tooltipEl);
  }

  return tooltipEl;
};

const externalTooltipHandler = (context) => {
  // Tooltip Element
  const {chart, tooltip} = context;
  const tooltipEl = getOrCreateTooltip(chart);

  // Hide if no tooltip
  if (tooltip.opacity === 0) {
    tooltipEl.style.opacity = 0;
    return;
  }

  // Set Text
  if (tooltip.body) {
    const titleLines = tooltip.title || [];
    const bodyLines = tooltip.body.map(b => b.lines);

    const tableHead = document.createElement('thead');

    titleLines.forEach(title => {
      const tr = document.createElement('tr');
      tr.style.borderWidth = 0;

      const th = document.createElement('th');
      th.style.borderWidth = 0;
      const text = document.createTextNode(title);

      th.appendChild(text);
      tr.appendChild(th);
      tableHead.appendChild(tr);
    });

    const tableBody = document.createElement('tbody');
    bodyLines.forEach((body, i) => {
      const colors = tooltip.labelColors[i];

      const span = document.createElement('span');
      span.style.background = colors.borderColor;
      span.style.borderColor = colors.borderColor;
      span.style.borderWidth = '2px';
      span.style.marginRight = '10px';
      span.style.height = '10px';
      span.style.width = '10px';
      span.style.display = 'inline-block';

      const tr = document.createElement('tr');
      tr.style.backgroundColor = 'inherit';
      tr.style.borderWidth = 0;

      const td = document.createElement('td');
      td.style.borderWidth = 0;

      const text = document.createTextNode(body);

      td.appendChild(span);
      td.appendChild(text);
      tr.appendChild(td);
      tableBody.appendChild(tr);
    });

    const tableRoot = tooltipEl.querySelector('table');

    // Remove old children
    while (tableRoot.firstChild) {
      tableRoot.firstChild.remove();
    }

    // Add new children
    tableRoot.appendChild(tableHead);
    tableRoot.appendChild(tableBody);
  }

  const {offsetLeft: positionX, offsetTop: positionY} = chart.canvas;

  // Display, position, and set styles for font
  tooltipEl.style.opacity = 1;
  tooltipEl.style.left = positionX + tooltip.caretX + 'px';
  tooltipEl.style.top = positionY + tooltip.caretY + 'px';
  tooltipEl.style.font = tooltip.options.bodyFont.string;
  tooltipEl.style.padding = tooltip.options.padding + 'px ' + tooltip.options.padding + 'px';
};

const Graph = (props) => {
  
    props.socket.onopen = function(e) {
      console.log("[open] Connection established");
    };

    props.socket.onclose = function(e) {
      console.log("[close] Connection closed");
    }
    
    props.socket.onmessage = function(event) {
      let obj = {};
      try{
        obj = JSON.parse(event.data);
      }catch (e){
        props.socket.send("Cannot Parse");
      }
      if(obj.op === "WRITE"){
        setAllCharts(current =>
          current.map(object => {
            if (object.id === obj.response.file) {
              return {...object, chartData: obj.response.data};
            }
            return object;
          }),
        );
      }else if(obj.op === "CREATE"){
        const option = {
          scales:{
            x: {
              ticks:{
                color:"white",
              },
              title:{
                display:false,
                text:'Steps'
              },
            },
            y: {
              ticks:{
                color:"white",
              },
              title: {
                display: false,
                text: 'Value'
              },
            },
          },
          transitions: {
            show: {
              animations: {
                x: {
                  from: 0
                },
                y: {
                  from: 0
                }
              }
            },
            hide: {
              animations: {
                x: {
                  to: 0
                },
                y: {
                  to: 1000
                }
              }
            }
          },
          elements:{
            line:{
              fill:true,
              tension:0.4,
              borderWidth:4,
            },
            point: {
              pointStyle: "circle",
              radius: 3,
            }
          },
          interaction: {
            intersect: false,
            mode: 'index',
          },
          hoverRadius: 12,
          maintainAspectRatio:false,
          responsive: true,
          hover: {
            mode: 'index',
            intersec: false
          },
          plugins:{
            filler: {
              propagate: true,
            },
            tooltip: {
              enabled: false,
              position: 'nearest',
              external: externalTooltipHandler
            },
            legend:{
              tooltip:false,
              position:"bottom",
              labels: {
                // This more specific font property overrides the global property
                font: function(context) {
                  var width = context.chart.width;
                  var size = 0.0;
                  if(width >=1024){
                    size = Math.round(width / 38);
                  }else if ( width > 480  && width < 1024){
                    size = Math.round(width / 32);
                  }else if ( width <= 480){
                    size = Math.round(width / 20);
                  }
                  return {
                    size: size,
                    weight: 600,
                  };
                },
            }
            },
            title:{
              color:"white",
              align:'center',
              display:true,
              font:{
                size:30,
              },
              text:obj.response.file,
            },
          },
        };
        console.log(obj.response);
        const local_storage = window.sessionStorage.getItem('colors');
        let localStorageColor = JSON.parse(local_storage,reviver);
        console.log(localStorageColor);
        let color = getOrCreateColors(obj.response.file,obj.response.data,localStorageColor);
        for(let i = 0; i < obj.response.data.datasets.length;i++){
          obj.response.data.datasets[i].backgroundColor = opacityRgba(color[i]);
          obj.response.data.datasets[i].borderColor = color[i];
        }
        const object = {
          id: obj.response.file,
          chartData: obj.response.data,
          chartOptions: option,
        };
        setAllCharts(current => [...current,object]);
      }else if(obj.op === "REMOVE"){
        setAllCharts(current =>
          current.filter(object => {
            return object.id !== obj.file;
          }),
        );
        const local_storage = window.sessionStorage.getItem('colors');
        let localStorageColor = JSON.parse(local_storage,reviver);
        removeChartColor(obj.file,localStorageColor);
      }
    };

  const [visible, setVisible] = useState(false)
  
  const toggleVisible = () => {
    const scrolled = document.documentElement.scrollTop;
    if (scrolled > 300){
      setVisible(true)
    } 
    else if (scrolled <= 300){
      setVisible(false)
    }
  };
  
  window.addEventListener('scroll', toggleVisible);
  const [allCharts,setAllCharts] = useState([]);
  const url = 'http://127.0.0.1:8000/getcsvdata';
  useEffect(() => {
    AOS.init();
    AOS.refresh();
    axios.get(url)
      .then((response)=>{
        console.log(response.data);
        let allChartsInput = [];
        if(JSON.parse(window.sessionStorage.getItem('colors')) === null){
          let color_object = new Map();
          for(let i=0; i<response.data.length; i++){
            let color_input = [];
            let id_input = response.data[i].file;
            for(let j=0;j<response.data[i].data.datasets.length;j++){
              let color = randomRgbaColor();
              response.data[i].data.datasets[j].backgroundColor = opacityRgba(color);
              response.data[i].data.datasets[j].borderColor = color;
              color_input.push(color);
            }
            color_object.set(id_input,color_input);
          }
          const str = JSON.stringify(color_object, replacer);
          window.sessionStorage.setItem('colors', JSON.stringify(color_object, replacer));
        }else{
          const local_storage = window.sessionStorage.getItem('colors');
          let localStorageColor = JSON.parse(local_storage,reviver);
          console.log(localStorageColor);
          for(let i=0; i<response.data.length; i++){
            let color = getOrCreateColors(response.data[i].file,response.data[i].data,localStorageColor);
            for(let j=0;j<response.data[i].data.datasets.length;j++){
              response.data[i].data.datasets[j].backgroundColor = opacityRgba(color[j]);
              response.data[i].data.datasets[j].borderColor = color[j];
            }
          }
        }
        for(let i=0; i<response.data.length; i++){
          const option = {
            scales:{
              x: {
                ticks:{
                  color:"white",
                },
                title:{
                  display:false,
                  text:'Steps'
                },
              },
              y: {
                ticks:{
                  color:"white",
                },
                title: {
                  display: false,
                  text: 'Value'
                },
              },
            },
            transitions: {
              show: {
                animations: {
                  x: {
                    from: 0
                  },
                  y: {
                    from: 0
                  }
                }
              },
              hide: {
                animations: {
                  x: {
                    to: 0
                  },
                  y: {
                    to: 1000
                  }
                }
              }
            },
            elements:{
              line:{
                fill:true,
                tension:0.4,
                borderWidth:4,
              },
              point: {
                pointStyle: "circle",
                radius: 3,
              }
            },
            interaction: {
              intersect: false,
              mode: 'index',
            },
            hoverRadius: 12,
            maintainAspectRatio:false,
            responsive: true,
            hover: {
              mode: 'index',
              intersec: false
            },
            plugins:{
              filler: {
                propagate: true,
              },
              tooltip: {
                enabled: false,
                position: 'nearest',
                external: externalTooltipHandler
              },
              legend:{
                tooltip:false,
                position:"bottom",
                labels: {
                  color:"white",
                  // This more specific font property overrides the global property
                  font: function(context) {
                    var width = context.chart.width;
                    var size = 0.0;
                    if(width >=1024){
                      size = Math.round(width / 38);
                    }else if ( width > 480  && width < 1024){
                      size = Math.round(width / 32);
                    }else if ( width <= 480){
                      size = Math.round(width / 20);
                    }
                    return {
                      size: size,
                      weight: 600,
                    };
                  },
              }
              },
              title:{
                color:"white",
                align:'center',
                display:true,
                font:{
                  size:30,
                },
                text:response.data[i].file,
              },
            },
            
          };
          let object = {
            id : response.data[i].file,
            chartData : response.data[i].data,
            chartOptions: option,
          }
          allChartsInput.push(object);
        }
        setAllCharts(allChartsInput);
      });
  }, []);
  return (
    <div>
      {Object.keys(allCharts).map(key =>
        <div
          data-aos="fade-up" 
          data-aos-duration="1500"
          className="chartContainer"
          style={{
            paddingTop:"30px",
            paddingBottom:"100px",
          }}
          key={allCharts[key].id}>
            <Line className="chart" data={allCharts[key].chartData} options={allCharts[key].chartOptions}/>
              <Link className="btn-one" style={{color:"white"}} to={"/Chart/"+allCharts[key].id} state={{data : allCharts[key].chartData,name:allCharts[key].id}}>
                  <span>SHOW MORE</span>
              </Link>
        </div>
    )}
    <button
        onClick={() => {
          window.scrollTo({top: 0, left: 0, behavior: 'smooth'});
        }}
        style={{
          display: visible ? 'inline' : 'none',
          position: 'fixed',
          padding: '1rem 2rem',
          fontSize: '20px',
          bottom: '40px',
          right: '40px',
          backgroundColor: 'grey',
          color: '#fff',
          textAlign: 'center',
        }}
      >
        <FaArrowUp />
      </button>
    </div>
  )
}

export default Graph