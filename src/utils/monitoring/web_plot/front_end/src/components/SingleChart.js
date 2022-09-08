import React from 'react'
import axios from 'axios'
import { useLocation, Link } from "react-router-dom";
import {useState, useEffect } from 'react'
import {FaArrowLeft} from 'react-icons/fa';
import { 
  ArcElement,
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
  registerables,
} from 'chart.js'
import {Chart} from 'react-chartjs-2';
import AOS from "aos";
import "aos/dist/aos.css";
  
ChartJS.register(
  ArcElement,
  Filler,
  LineElement,
  CategoryScale,
  LinearScale,
  BarElement,
  PointElement,
  Title,
  Tooltip,
  Legend,
  ...registerables,
);


function getColors(id,data,localStorageColor){
  console.log(id);
  console.log(data);
  console.log(localStorageColor);
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

function opacityRgba(rgba) {
  return rgba.replace("1)","0.1)")
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

const SingleChart = (props) => {
const location = useLocation();
let location_state = location.state;
const [chartData,setChartData] = useState(location_state.data);


var tempLabels = [];
var tempData = [];
var bgColor = [];
var bdColor = [];
const [dogData,setDogData] = useState({
  labels: tempLabels,
  datasets: [
    {
      backgroundColor:bgColor,
      borderColor:bdColor,
      data: tempData,
    }
  ]
});

const url = 'http://127.0.0.1:8000/buildsingledata/'+location_state.name;
useEffect (() => {
  AOS.init();
  AOS.refresh();
  window.scrollTo({top: 0, left: 0, behavior: 'smooth'});
  axios.get(url)
    .then((response)=>{
      console.log(response);
      const local_storage = window.sessionStorage.getItem('colors');
      let localStorageColor = JSON.parse(local_storage,reviver);
      let colors = getColors(response.data[0].file,response.data[0].data,localStorageColor);
      for(var j = 0; j < response.data[0].data.datasets.length; j++){
        tempLabels.push(response.data[0].data.datasets[j].label);
        tempData.push(response.data[0].data.datasets[j].data[response.data[0].data.datasets[j].data.length-1]);
        bgColor.push(opacityRgba(colors[j]));
        bdColor.push(colors[j]);
      }
      setChartData(response.data[0].data);
      setDogData({
        labels: tempLabels,
        datasets: [
          {
            backgroundColor:bgColor,
            borderColor:bdColor,
            data: tempData,
          }
        ]
      });
    })
  },[]);




const totalDuration = 2000;
const delayBetweenPoints = totalDuration / location_state.data.labels.length;
const previousY = (ctx) => ctx.index === 0 ? ctx.chart.scales.y.getPixelForValue(100) : ctx.chart.getDatasetMeta(ctx.datasetIndex).data[ctx.index - 1].getProps(['y'], true).y;
const animation = {
  x: {
    type: 'number',
    easing: 'linear',
    duration: delayBetweenPoints,
    from: NaN, // the point is initially skipped
    delay(ctx) {
      if (ctx.type !== 'data' || ctx.xStarted) {
        return 0;
      }
      ctx.xStarted = true;
      return ctx.index * delayBetweenPoints;
    }
  },
  y: {
    type: 'number',
    easing: 'linear',
    duration: delayBetweenPoints,
    from: previousY,
    delay(ctx) {
      if (ctx.type !== 'data' || ctx.yStarted) {
        return 0;
      }
      ctx.yStarted = true;
      return ctx.index * delayBetweenPoints;
    }
  }

  
};

const [dougChartOptions,setDougChartOptions] = useState({
  plugins:{
    tooltip: {
      enabled: false,
      position: 'nearest',
      external: externalTooltipHandler
    },
    legend:{
      labels:{
        color:"white",
        // This more specific font property overrides the global property
        font: function(context) {
          var width = context.chart.width;
          var size = 0.0;
          if(width >=1024){
            size = Math.round(width / 50);
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
        padding:20,
        usePointStyle:true,
      },
      display:true,
      position:"bottom",
      align:'center',
    },
  },
  maintainAspectRatio:false,
  responsive:true,
});

  const [barChartOptions,setBarChartOptions] = useState({
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
    elements:{
      bar:{
        fill:true,
        tension:0.4,
        borderWidth:3,
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
    hover: {
      mode: 'index',
      intersec: false
    },
    plugins:{
      legend:{
        display:false,
        position:"bottom",

      },
      tooltip: {
        enabled: false,
        position: 'nearest',
        external: externalTooltipHandler
      },
    },
    maintainAspectRatio:false,
    responsive:true,
  });
  var hoveredDatasetIndex = -1;
  const [lineChartOptions,setLineChartOptions] = useState({
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
    interaction: {
      intersect: false,
      mode: 'index',
    },
    animation: animation,
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
              size = Math.round(width / 50);
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
      
    },
  });

  props.socket.onopen = function(e) {
    console.log("[open] Connection established");
    props.socket.send("ciao");

  };

  props.socket.onmessage = function(event) {
    console.log(`[message] Data received from server: ${event.data}`);
    let obj = {};
    try{
      obj = JSON.parse(event.data);
    }catch (e){
      console.log(e);
      props.socket.send("Cannot Parse");
    }
    const local_storage = window.sessionStorage.getItem('colors');
    let localStorageColor = JSON.parse(local_storage,reviver);
    if(obj.op === "WRITE"){
      tempLabels.length = 0
      tempData.length = 0
      bgColor.length = 0
      bdColor.length = 0
      if(location_state.name === obj.response.file){
        let colors = getColors(obj.response.file,obj.response.data,localStorageColor);
        for(var j = 0; j < obj.response.data.datasets.length; j++){
          tempLabels.push(obj.response.data.datasets[j].label);
          var x = obj.response.data.datasets[j].data[obj.response.data.datasets[j].data.length-1];
          tempData.push(x);
          bgColor.push(opacityRgba(colors[j]));
          bdColor.push(colors[j]);
        }
        setDogData({
          labels: tempLabels,
          datasets: [
            {
              backgroundColor:bgColor,
              borderColor:bdColor,
              label: "ciao",
              data: tempData,
            }
          ]
        });
        console.log(obj.response.data);
        setChartData(obj.response.data);
        var newDogData = [];
        for(var k = 0; k < obj.response.data.datasets.length; k++){
          newDogData.push(obj.response.data.datasets[k].data[obj.response.data.datasets[k].data.length-1]);
        }
        setDogData({
          labels: tempLabels,
          datasets: [
            {
              backgroundColor:bgColor,
              borderColor:bdColor,
              label: "ciao",
              data: newDogData,
            }
          ]
        });
      }
    }else if(obj.op === "REMOVE"){
      if(location_state.name === obj.file){
        removeChartColor(obj.file,localStorageColor);
        window.location.replace("/");
    }
  }
  }

  props.socket.onclose = function(e) {
    console.log("[close] Connection closed");
  }
  
  return (
    <div>
      <div data-aos="flip-up" data-aos-duration="1000">
      <button
        onClick={() => {
          window.location.replace("/");
        }}
        className="back-button"
      >
        <FaArrowLeft />
      </button>
      <h1 style={{color:"white"}}> Resuming info for {location_state.name}</h1>
      </div>
      <div data-aos="fade-right" data-aos-delay="500" data-aos-duration="1000" className="InfoContainer">
        <div className="LineContainer">
          <Chart type="line" data={chartData} options={lineChartOptions}/>
        </div>
        <div data-aos="fade-left" data-aos-delay="1000" data-aos-duration="1000" className="OtherContainer">
          <div className="DougContainer">
            <h1 style={{color:"white"}}>Final Values</h1>
            <Chart type="doughnut" data={dogData} options={dougChartOptions}/>
          </div>
          <div className="BarContainer">
            <Chart type="bar" data={chartData} options={barChartOptions}/>
          </div>
        </div>
      </div>
    </div>
  )
}

export default SingleChart