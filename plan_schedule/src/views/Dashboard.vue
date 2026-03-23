<template>
  <div class="dashboard">
    <div class="container">
      <h1>大学生时间规划</h1>
      
      <!-- 快捷入口 -->
      <div class="quick-access">
        <div class="quick-item" @click="$router.push('/courses')">
          <div class="quick-icon courses">📚</div>
          <div class="quick-title">课程管理</div>
        </div>
        <div class="quick-item" @click="$router.push('/tasks')">
          <div class="quick-icon tasks">✅</div>
          <div class="quick-title">任务规划</div>
        </div>
        <div class="quick-item" @click="$router.push('/goals')">
          <div class="quick-icon goals">🎯</div>
          <div class="quick-title">目标管理</div>
        </div>
        <div class="quick-item" @click="$router.push('/focus')">
          <div class="quick-icon focus">⏰</div>
          <div class="quick-title">专注模式</div>
        </div>
      </div>
      
      <!-- 今日概览 -->
      <div class="card">
        <h2>今日概览</h2>
        <div class="today-overview">
          <div class="overview-item">
            <div class="overview-label">今日课程</div>
            <div class="overview-value">{{ todayCourses.length }}</div>
            <div v-if="todayCourses.length > 0" class="overview-detail">
              <div v-for="course in todayCourses" :key="course.id" class="course-item">
                {{ course.name }} {{ course.time }}
              </div>
            </div>
          </div>
          <div class="overview-item">
            <div class="overview-label">待办任务</div>
            <div class="overview-value">{{ pendingTasks.length }}</div>
            <div v-if="pendingTasks.length > 0" class="overview-detail">
              <div v-for="task in pendingTasks" :key="task.id" class="task-item">
                {{ task.title }}
              </div>
            </div>
          </div>
          <div class="overview-item">
            <div class="overview-label">进行中目标</div>
            <div class="overview-value">{{ activeGoals.length }}</div>
            <div v-if="activeGoals.length > 0" class="overview-detail">
              <div v-for="goal in activeGoals" :key="goal.id" class="goal-item">
                {{ goal.title }}
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 最近活动 -->
      <div class="card">
        <h2>最近活动</h2>
        <div class="activity-list">
          <div v-for="activity in recentActivities" :key="activity.id" class="activity-item">
            <div class="activity-time">{{ activity.time }}</div>
            <div class="activity-content">{{ activity.content }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

// 模拟数据
const todayCourses = ref([
  { id: 1, name: '高等数学', time: '08:00-09:40' },
  { id: 2, name: '大学英语', time: '14:00-15:40' }
])

const pendingTasks = ref([
  { id: 1, title: '完成数学作业' },
  { id: 2, title: '准备英语演讲' }
])

const activeGoals = ref([
  { id: 1, title: '英语四级备考' },
  { id: 2, title: '期末复习' }
])

const recentActivities = ref([
  { id: 1, time: '今天 08:00', content: '开始高等数学课程' },
  { id: 2, time: '昨天 16:00', content: '完成编程作业' },
  { id: 3, time: '昨天 14:00', content: '参加英语听力练习' }
])
</script>

<style scoped>
.dashboard {
  padding: 20px 0;
}

h1 {
  text-align: center;
  margin-bottom: 30px;
  color: var(--primary-color);
}

.quick-access {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 20px;
  margin-bottom: 30px;
}

.quick-item {
  background-color: var(--white);
  border-radius: var(--border-radius);
  box-shadow: var(--card-shadow);
  padding: 20px;
  text-align: center;
  cursor: pointer;
  transition: var(--transition);
}

.quick-item:hover {
  transform: translateY(-5px);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
}

.quick-icon {
  font-size: 48px;
  margin-bottom: 10px;
}

.quick-title {
  font-size: 16px;
  font-weight: 500;
}

.today-overview {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 20px;
}

.overview-item {
  padding: 20px;
  background-color: #f9f9f9;
  border-radius: var(--border-radius);
}

.overview-label {
  font-size: 14px;
  color: var(--text-color-secondary);
  margin-bottom: 10px;
}

.overview-value {
  font-size: 32px;
  font-weight: bold;
  color: var(--primary-color);
  margin-bottom: 10px;
}

.overview-detail {
  font-size: 12px;
  color: var(--text-color-secondary);
}

.course-item,
.task-item,
.goal-item {
  margin-bottom: 5px;
}

.activity-list {
  margin-top: 20px;
}

.activity-item {
  display: flex;
  margin-bottom: 15px;
  padding-bottom: 15px;
  border-bottom: 1px solid var(--border-color);
}

.activity-item:last-child {
  border-bottom: none;
  margin-bottom: 0;
  padding-bottom: 0;
}

.activity-time {
  width: 100px;
  color: var(--text-color-secondary);
  font-size: 12px;
}

.activity-content {
  flex: 1;
}

@media (max-width: 768px) {
  .quick-access {
    grid-template-columns: repeat(2, 1fr);
  }
  
  .today-overview {
    grid-template-columns: 1fr;
  }
  
  .activity-item {
    flex-direction: column;
  }
  
  .activity-time {
    width: 100%;
    margin-bottom: 5px;
  }
}
</style>