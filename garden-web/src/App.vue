<template>
  <div id="app">
    <h1>Home Garden Moments</h1>
    <p>GardenCore 核心逻辑实现完成</p>
    <div class="demo">
      <h2>播种测试</h2>
      <el-button type="primary" @click="testPlant">播种番茄</el-button>
      <div v-if="plotState" class="result">
        <h3>地块状态</h3>
        <p>地块 ID: {{ plotState.id }}</p>
        <p>作物编码: {{ plotState.crop_code }}</p>
        <p>阶段: {{ plotState.stage }}</p>
        <p>成长: {{ plotState.growth.toFixed(2) }}%</p>
        <p>变体: {{ plotState.variant }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

// TODO: 集成 garden-wasm 或 garden-native
const plotState = ref<any>(null)

const testPlant = async () => {
  try {
    // 浏览器环境：使用 WASM
    // @ts-ignore
    const gardenWasm = new GardenWasm()
    const state = await gardenWasm.plant(1, 'tomato_v1', 1, null)
    plotState.value = state
  } catch (error) {
    console.error('播种失败:', error)
    alert('播种失败: ' + error)
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}

.demo {
  margin-top: 40px;
}

.result {
  margin-top: 20px;
  text-align: left;
  padding: 20px;
  background: #f5f5f5;
  border-radius: 8px;
}
</style>
