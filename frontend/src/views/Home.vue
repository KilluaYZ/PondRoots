
<template>
  <kinesis-container>
    <el-row style="width: 100%; height: 100%;display: flex; justify-content: center; align-items: center; flex-direction: column;overflow-y: auto">
      <el-row style="display: flex; flex-direction: column; justify-content: space-between; align-items: center; width: 78%; height: 90%">
        <el-row style="display: flex; flex-direction: row; justify-content: center; align-items: center; width: 100%; height: 80%;">
          <el-row style="height: 100%; width: 50%; display: flex; justify-content: center; align-items: center; flex-wrap: wrap">
            <el-row style="height: 100%; width: fit-content; display: flex; flex-direction: column; justify-content: center; flex-wrap: nowrap; align-items: start">
              <kinesis-element style="width: fit-content; height: fit-content; padding: 0 10px 0 10px"><span style="font-weight: bold; font-size: xxx-large; color: #081642">Pond Memory</span></kinesis-element>
              <span style="font-weight: normal; font-size: large; color: #081642; margin: 0 10px 25px 10px">小池塘的回忆，收藏我们的每一天</span>
              <el-row style="width: 100%;height: fit-content; display: flex; flex-direction: row; justify-content: start; align-items: center; margin: 0 10px 0 10px">
                <el-button class="home_page_btn home_page_btn1" round type="primary" size="large">进入回忆</el-button>
                <el-button class="home_page_btn home_page_btn2" round type="primary" size="large">查看更多</el-button>
              </el-row>
            </el-row>
          </el-row>
          <el-row ID="control-bar" style="height: 100%; width: 50%; display: flex; justify-content: center; align-items: center">
<!--            <Echarts :option="option" height="100%" width="100%" @click="onClickItem(option)" />-->
          </el-row>
        </el-row>
        <el-row style="display: flex; flex-direction: row; justify-content: center; align-items: center; width: 100%; height: 10%">
          <el-row style="width: 100%; height: 100%; display: flex; justify-content: center; align-items: center">
            <span style="font-size: 16px">📣访问</span>
            <el-link type="primary" @click="onClickElLink" style="font-size: 16px">代码仓库</el-link>
            <span style="font-size: 16px">获取更多信息~</span>
          </el-row>
        </el-row>
      </el-row>
    </el-row>
  </kinesis-container>


</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import * as echarts from 'echarts';
import { onMounted } from 'vue'


const option = reactive({
  legend: {
    top: '5%',
    left: 'center',
    show: false
  },
  series: [
    {
      name: 'Access From',
      type: 'pie',
      radius: ['40%', '70%'],
      avoidLabelOverlap: false,
      itemStyle: {
        borderRadius: 10,
        borderColor: '#fff',
        borderWidth: 2
      },
      label: {
        show: false,
        position: 'center'
      },
      emphasis: {
        label: {
          show: true,
          fontSize: 35,
          fontWeight: 'bold'
        }
      },
      labelLine: {
        show: false
      },
      data: [
        { value: 1048, name: 'WechatMemory' },
        { value: 735, name: 'PhotoWall' },
        { value: 580, name: 'TravallingMap' },
        { value: 484, name: 'Books&Movies' },
        { value: 300, name: 'love u' }
      ]
    }
  ]
});

function init_func()
{
  var chartDom = document.getElementById('control-bar');
  var myChart = echarts.init(chartDom);
  myChart.on('click', function (params) {
    console.log(params.name)
    window.location.replace('http://localhost:2000/'+encodeURIComponent(params.name));
  });
  option && myChart.setOption(option);
}

const onClickElLink = () => {
  // shell.openExternal("https://gitee.com/killuayz/pond-memory");
  // openDefaultBrowser("https://gitee.com/killuayz/pond-memory");
  // window.electron.ipcRenderer.send('openProjectWindow');
  window.open('https://gitee.com/killuayz/pond-memory');
}

onMounted(init_func);



</script>


<style scoped>
.home_page_btn{
}

.home_page_btn1 {

}

.home_page_btn2{
  --el-button-bg-color: #ffffff;
  --el-button-text-color: #3f3f3f;

}

</style>
