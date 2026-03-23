<template>
  <div class="analytics">
    <div class="container">
      <h2>数据洞察</h2>
      
      <!-- 数据概览 -->
      <div class="card">
        <h3>数据概览</h3>
        <div class="stats-grid">
          <div class="stat-item">
            <div class="stat-value">{{ totalFocusTime }}</div>
            <div class="stat-label">总专注时长（小时）</div>
          </div>
          <div class="stat-item">
            <div class="stat-value">{{ taskCompletionRate }}%</div>
            <div class="stat-label">任务完成率</div>
          </div>
          <div class="stat-item">
            <div class="stat-value">{{ goalCompletionRate }}%</div>
            <div class="stat-label">目标完成率</div>
          </div>
          <div class="stat-item">
            <div class="stat-value">{{ averageFocusSession }}</div>
            <div class="stat-label">平均专注时长（分钟）</div>
          </div>
        </div>
      </div>
      
      <!-- 时间分布图表 -->
      <div class="card">
        <h3>时间分布</h3>
        <div class="chart-container">
          <canvas ref="timeDistributionChart"></canvas>
        </div>
      </div>
      
      <!-- 任务完成趋势 -->
      <div class="card">
        <h3>任务完成趋势</h3>
        <div class="chart-container">
          <canvas ref="taskCompletionChart"></canvas>
        </div>
      </div>
      
      <!-- 专注时长趋势 -->
      <div class="card">
        <h3>专注时长趋势</h3>
        <div class="chart-container">
          <canvas ref="focusTimeChart"></canvas>
        </div>
      </div>
      
      <!-- 智能分析 -->
      <div class="card">
        <h3>智能分析</h3>
        <div class="analysis-content">
          <div class="analysis-item">
            <div class="analysis-icon">📈</div>
            <div class="analysis-text">
              <h4>高效时间段</h4>
              <p>您在{{ mostProductiveTime }}的专注效率最高，建议在这个时间段安排重要任务。</p>
            </div>
          </div>
          <div class="analysis-item">
            <div class="analysis-icon">⚠️</div>
            <div class="analysis-text">
              <h4>改进建议</h4>
              <p>您的{{ mostOverdueTaskType }}任务逾期率较高，建议提高该类型任务的优先级。</p>
            </div>
          </div>
          <div class="analysis-item">
            <div class="analysis-icon">💡</div>
            <div class="analysis-text">
              <h4>时间管理建议</h4>
              <p>本周您的平均专注时长为{{ averageWeeklyFocus }}小时，建议保持并适当增加。</p>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 数据导出 -->
      <div class="card">
        <h3>数据导出</h3>
        <div class="export-actions">
          <button class="btn btn-primary">导出时间使用数据</button>
          <button class="btn btn-success">导出任务完成记录</button>
          <button class="btn btn-warning">导出专注时长报表</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { Chart, registerables } from 'chart.js'

// 注册Chart.js组件
Chart.register(...registerables)

// 数据概览
const totalFocusTime = ref(120) // 总专注时长120小时
const taskCompletionRate = ref(85) // 任务完成率85%
const goalCompletionRate = ref(70) // 目标完成率70%
const averageFocusSession = ref(25) // 平均专注时长25分钟

// 智能分析数据
const mostProductiveTime = ref('上午9:00-11:00')
const mostOverdueTaskType = ref('作业')
const averageWeeklyFocus = ref(14) // 平均每周专注14小时

// 图表引用
const timeDistributionChart = ref(null)
const taskCompletionChart = ref(null)
const focusTimeChart = ref(null)

// 图表实例
let timeDistributionChartInstance = null
let taskCompletionChartInstance = null
let focusTimeChartInstance = null

// 组件挂载时初始化图表
onMounted(() => {
  initCharts()
})

// 初始化图表
const initCharts = () => {
  // 时间分布图表
  if (timeDistributionChart.value) {
    timeDistributionChartInstance = new Chart(timeDistributionChart.value, {
      type: 'pie',
      data: {
        labels: ['课程学习', '自主学习', '休息娱乐', '其他'],
        datasets: [{
          data: [40, 30, 20, 10],
          backgroundColor: [
            '#409EFF',
            '#67C23A',
            '#E6A23C',
            '#909399'
          ],
          borderWidth: 0
        }]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
          legend: {
            position: 'bottom'
          }
        }
      }
    })
  }
  
  // 任务完成趋势图表
  if (taskCompletionChart.value) {
    taskCompletionChartInstance = new Chart(taskCompletionChart.value, {
      type: 'line',
      data: {
        labels: ['周一', '周二', '周三', '周四', '周五', '周六', '周日'],
        datasets: [{
          label: '完成任务数',
          data: [5, 6, 4, 7, 5, 8, 6],
          borderColor: '#409EFF',
          backgroundColor: 'rgba(64, 158, 255, 0.1)',
          tension: 0.4,
          fill: true
        }]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        scales: {
          y: {
            beginAtZero: true
          }
        }
      }
    })
  }
  
  // 专注时长趋势图表
  if (focusTimeChart.value) {
    focusTimeChartInstance = new Chart(focusTimeChart.value, {
      type: 'bar',
      data: {
        labels: ['周一', '周二', '周三', '周四', '周五', '周六', '周日'],
        datasets: [{
          label: '专注时长（分钟）',
          data: [120, 150, 90, 180, 120, 240, 180],
          backgroundColor: '#67C23A'
        }]
      },
      options: {
        responsive: true,
        maintainAspectRatio: false,
        scales: {
          y: {
            beginAtZero: true
          }
        }
      }
    })
  }
}
</script>

<style scoped>
.analytics {
  padding: 20px 0;
}

h2 {
  margin-bottom: 20px;
  color: var(--primary-color);
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

.chart-container {
  height: 300px;
  margin-top: 20px;
}

.analysis-content {
  margin-top: 20px;
}

.analysis-item {
  display: flex;
  margin-bottom: 20px;
  padding: 16px;
  background-color: #f9f9f9;
  border-radius: var(--border-radius);
}

.analysis-icon {
  font-size: 32px;
  margin-right: 16px;
  width: 50px;
  text-align: center;
}

.analysis-text h4 {
  margin: 0 0 8px 0;
  font-size: 16px;
  font-weight: 500;
}

.analysis-text p {
  margin: 0;
  font-size: 14px;
  color: var(--text-color-secondary);
  line-height: 1.5;
}

.export-actions {
  margin-top: 20px;
  display: flex;
  gap: 10px;
}

@media (max-width: 768px) {
  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }
  
  .analysis-item {
    flex-direction: column;
    text-align: center;
  }
  
  .analysis-icon {
    margin-right: 0;
    margin-bottom: 12px;
  }
  
  .export-actions {
    flex-direction: column;
  }
  
  .export-actions button {
    width: 100%;
  }
}
</style>