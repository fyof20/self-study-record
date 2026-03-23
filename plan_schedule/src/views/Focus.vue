<template>
  <div class="focus">
    <div class="container">
      <h2>专注模式</h2>
      
      <!-- 专注计时器 -->
      <div class="card focus-timer">
        <h3>专注计时器</h3>
        <div class="timer-container">
          <div class="timer-display">{{ formattedTime }}</div>
          <div class="timer-controls">
            <button class="btn btn-primary" @click="toggleTimer">
              {{ isRunning ? '暂停' : '开始' }}
            </button>
            <button class="btn btn-warning" @click="resetTimer">重置</button>
            <button class="btn btn-danger" @click="stopTimer">停止</button>
          </div>
        </div>
        
        <!-- 专注设置 -->
        <div class="focus-settings">
          <h4>专注设置</h4>
          <div class="settings-grid">
            <div class="setting-item">
              <label class="form-label">专注时长（分钟）</label>
              <input type="number" class="form-control" v-model.number="focusTime" :disabled="isRunning">
            </div>
            <div class="setting-item">
              <label class="form-label">休息时长（分钟）</label>
              <input type="number" class="form-control" v-model.number="breakTime" :disabled="isRunning">
            </div>
            <div class="setting-item">
              <label class="form-label">循环次数</label>
              <input type="number" class="form-control" v-model.number="cycles" :disabled="isRunning">
            </div>
            <div class="setting-item">
              <label class="form-label">专注模式</label>
              <select class="form-control" v-model="focusMode" :disabled="isRunning">
                <option value="pomodoro">番茄工作法</option>
                <option value="custom">自定义计时</option>
                <option value="countup">正计时</option>
              </select>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 专注统计 -->
      <div class="card focus-stats">
        <h3>专注统计</h3>
        <div class="stats-grid">
          <div class="stat-item">
            <div class="stat-value">{{ todayFocusTime }}</div>
            <div class="stat-label">今日专注时长（分钟）</div>
          </div>
          <div class="stat-item">
            <div class="stat-value">{{ weekFocusTime }}</div>
            <div class="stat-label">本周专注时长（分钟）</div>
          </div>
          <div class="stat-item">
            <div class="stat-value">{{ totalFocusSessions }}</div>
            <div class="stat-label">总专注次数</div>
          </div>
          <div class="stat-item">
            <div class="stat-value">{{ averageFocusTime }}</div>
            <div class="stat-label">平均专注时长（分钟）</div>
          </div>
        </div>
      </div>
      
      <!-- 场景化专注 -->
      <div class="card focus-scenes">
        <h3>场景化专注</h3>
        <div class="scenes-list">
          <div v-for="scene in focusScenes" :key="scene.id" class="scene-item" @click="selectScene(scene)">
            <div class="scene-icon">{{ scene.icon }}</div>
            <div class="scene-name">{{ scene.name }}</div>
            <div class="scene-selected" v-if="selectedScene === scene.id">✓</div>
          </div>
        </div>
      </div>
      
      <!-- 组队云自习 -->
      <div class="card focus-room">
        <h3>组队云自习</h3>
        <div class="room-actions">
          <button class="btn btn-primary">创建房间</button>
          <button class="btn btn-success">加入房间</button>
        </div>
        <div class="room-list">
          <div v-for="room in focusRooms" :key="room.id" class="room-item">
            <div class="room-name">{{ room.name }}</div>
            <div class="room-info">
              <span>{{ room.members }}人</span>
              <span>{{ room.focusTime }}分钟</span>
            </div>
            <button class="btn btn-sm btn-primary">加入</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted } from 'vue'

// 计时器状态
const isRunning = ref(false)
const isPaused = ref(false)
const timeLeft = ref(25 * 60) // 默认25分钟
const focusTime = ref(25)
const breakTime = ref(5)
const cycles = ref(4)
const focusMode = ref('pomodoro')
const currentCycle = ref(1)
const isBreak = ref(false)

// 专注统计
const todayFocusTime = ref(120) // 今日专注2小时
const weekFocusTime = ref(840) // 本周专注14小时
const totalFocusSessions = ref(48)
const averageFocusTime = ref(25)

// 专注场景
const focusScenes = ref([
  { id: 1, name: '图书馆', icon: '📚' },
  { id: 2, name: '自习室', icon: '🏫' },
  { id: 3, name: '雨天', icon: '🌧️' },
  { id: 4, name: '白噪音', icon: '🔊' }
])
const selectedScene = ref(1)

// 云自习房间
const focusRooms = ref([
  { id: 1, name: '考研冲刺', members: 8, focusTime: 120 },
  { id: 2, name: '四六级备考', members: 5, focusTime: 90 },
  { id: 3, name: '期末复习', members: 12, focusTime: 150 }
])

// 计时器interval
let timerInterval = null

// 格式化时间显示
const formattedTime = computed(() => {
  const minutes = Math.floor(timeLeft.value / 60)
  const seconds = timeLeft.value % 60
  return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`
})

// 开始/暂停计时器
const toggleTimer = () => {
  if (isRunning.value) {
    clearInterval(timerInterval)
    isPaused.value = true
  } else {
    startTimer()
  }
  isRunning.value = !isRunning.value
}

// 开始计时器
const startTimer = () => {
  timerInterval = setInterval(() => {
    if (timeLeft.value > 0) {
      timeLeft.value--
    } else {
      handleTimerComplete()
    }
  }, 1000)
}

// 处理计时器完成
const handleTimerComplete = () => {
  clearInterval(timerInterval)
  isRunning.value = false
  
  if (isBreak.value) {
    // 休息结束，开始新的专注
    isBreak.value = false
    timeLeft.value = focusTime.value * 60
    if (currentCycle.value < cycles.value) {
      currentCycle.value++
      // 这里可以添加提醒
    } else {
      // 所有循环完成
      currentCycle.value = 1
      // 这里可以添加提醒
    }
  } else {
    // 专注结束，开始休息
    isBreak.value = true
    timeLeft.value = breakTime.value * 60
    // 这里可以添加提醒
  }
}

// 重置计时器
const resetTimer = () => {
  clearInterval(timerInterval)
  isRunning.value = false
  isPaused.value = false
  isBreak.value = false
  currentCycle.value = 1
  timeLeft.value = focusTime.value * 60
}

// 停止计时器
const stopTimer = () => {
  clearInterval(timerInterval)
  isRunning.value = false
  isPaused.value = false
  isBreak.value = false
  currentCycle.value = 1
  timeLeft.value = focusTime.value * 60
}

// 选择专注场景
const selectScene = (scene) => {
  selectedScene.value = scene.id
  // 这里可以添加场景音效切换逻辑
}

// 组件挂载时初始化
onMounted(() => {
  timeLeft.value = focusTime.value * 60
})

// 组件卸载时清理
onUnmounted(() => {
  if (timerInterval) {
    clearInterval(timerInterval)
  }
})
</script>

<style scoped>
.focus {
  padding: 20px 0;
}

h2 {
  margin-bottom: 20px;
  color: var(--primary-color);
}

.focus-timer {
  text-align: center;
}

.timer-container {
  margin: 40px 0;
}

.timer-display {
  font-size: 72px;
  font-weight: bold;
  color: var(--primary-color);
  margin-bottom: 30px;
}

.timer-controls {
  display: flex;
  justify-content: center;
  gap: 20px;
}

.focus-settings {
  margin-top: 40px;
  text-align: left;
}

.settings-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 20px;
  margin-top: 20px;
}

.focus-stats {
  margin-top: 20px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 20px;
  margin-top: 20px;
}

.stat-item {
  text-align: center;
  padding: 20px;
  background-color: #f9f9f9;
  border-radius: var(--border-radius);
}

.stat-value {
  font-size: 32px;
  font-weight: bold;
  color: var(--primary-color);
  margin-bottom: 8px;
}

.stat-label {
  font-size: 14px;
  color: var(--text-color-secondary);
}

.focus-scenes {
  margin-top: 20px;
}

.scenes-list {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 20px;
  margin-top: 20px;
}

.scene-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 20px;
  background-color: white;
  border: 2px solid var(--border-color);
  border-radius: var(--border-radius);
  cursor: pointer;
  transition: var(--transition);
  position: relative;
}

.scene-item:hover {
  border-color: var(--primary-color);
  transform: translateY(-5px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.scene-icon {
  font-size: 32px;
  margin-bottom: 10px;
}

.scene-name {
  font-size: 14px;
  font-weight: 500;
}

.scene-selected {
  position: absolute;
  top: 10px;
  right: 10px;
  color: var(--success-color);
  font-size: 16px;
  font-weight: bold;
}

.focus-room {
  margin-top: 20px;
}

.room-actions {
  margin-bottom: 20px;
  display: flex;
  gap: 10px;
}

.room-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.room-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px;
  background-color: white;
  border: 1px solid var(--border-color);
  border-radius: var(--border-radius);
}

.room-name {
  font-weight: 500;
}

.room-info {
  display: flex;
  gap: 20px;
  font-size: 14px;
  color: var(--text-color-secondary);
}

@media (max-width: 768px) {
  .timer-display {
    font-size: 48px;
  }
  
  .timer-controls {
    flex-direction: column;
    align-items: center;
  }
  
  .settings-grid {
    grid-template-columns: 1fr;
  }
  
  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }
  
  .scenes-list {
    grid-template-columns: repeat(2, 1fr);
  }
  
  .room-actions {
    flex-direction: column;
  }
  
  .room-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 10px;
  }
  
  .room-actions button {
    width: 100%;
  }
}
</style>