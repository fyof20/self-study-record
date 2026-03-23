<template>
  <div class="goals">
    <div class="container">
      <h2>目标管理</h2>
      
      <!-- 目标操作栏 -->
      <div class="goal-actions">
        <button class="btn btn-primary" @click="showAddGoalDialog = true">添加目标</button>
        <button class="btn btn-success">模板库</button>
        <button class="btn btn-warning">目标复盘</button>
      </div>
      
      <!-- 目标列表 -->
      <div class="card">
        <h3>我的目标</h3>
        <div class="goal-list">
          <div v-for="goal in goals" :key="goal.id" class="goal-card">
            <div class="goal-header" :style="{ backgroundColor: goal.color }">
              <h4>{{ goal.title }}</h4>
              <span class="goal-type">{{ getGoalTypeText(goal.type) }}</span>
            </div>
            <div class="goal-body">
              <div class="goal-info">
                <span class="info-item">📅 开始时间: {{ goal.startDate }}</span>
                <span class="info-item">📅 结束时间: {{ goal.endDate }}</span>
                <span class="info-item">🎯 目标进度: {{ goal.progress }}%</span>
              </div>
              <div class="goal-description">{{ goal.description }}</div>
              <div class="goal-progress">
                <div class="progress-bar">
                  <div class="progress-fill" :style="{ width: goal.progress + '%', backgroundColor: goal.color }"></div>
                </div>
              </div>
              <div class="goal-actions">
                <button class="btn btn-sm btn-primary">编辑</button>
                <button class="btn btn-sm btn-success" @click="updateGoalProgress(goal, 10)">+10%</button>
                <button class="btn btn-sm btn-danger" @click="deleteGoal(goal.id)">删除</button>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 目标模板 -->
      <div class="card">
        <h3>目标模板</h3>
        <div class="template-list">
          <div v-for="template in goalTemplates" :key="template.id" class="template-card">
            <div class="template-icon">{{ template.icon }}</div>
            <div class="template-content">
              <h4>{{ template.title }}</h4>
              <p>{{ template.description }}</p>
              <button class="btn btn-sm btn-primary" @click="useTemplate(template)">使用模板</button>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 添加目标对话框 -->
      <div v-if="showAddGoalDialog" class="dialog-overlay" @click="showAddGoalDialog = false">
        <div class="dialog-content" @click.stop>
          <h3>添加目标</h3>
          <div class="form-group">
            <label class="form-label">目标标题</label>
            <input type="text" class="form-control" v-model="newGoal.title">
          </div>
          <div class="form-group">
            <label class="form-label">目标描述</label>
            <textarea class="form-control" v-model="newGoal.description" rows="3"></textarea>
          </div>
          <div class="form-group">
            <label class="form-label">目标类型</label>
            <select class="form-control" v-model="newGoal.type">
              <option value="long">长期目标</option>
              <option value="medium">中期目标</option>
              <option value="short">短期目标</option>
            </select>
          </div>
          <div class="form-group">
            <label class="form-label">开始时间</label>
            <input type="date" class="form-control" v-model="newGoal.startDate">
          </div>
          <div class="form-group">
            <label class="form-label">结束时间</label>
            <input type="date" class="form-control" v-model="newGoal.endDate">
          </div>
          <div class="form-group">
            <label class="form-label">目标颜色</label>
            <input type="color" class="form-control" v-model="newGoal.color">
          </div>
          <div class="dialog-actions">
            <button class="btn btn-primary" @click="addGoal">确定</button>
            <button class="btn" @click="showAddGoalDialog = false">取消</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

// 模拟数据
const goals = ref([
  { id: 1, title: '英语四级备考', description: '通过英语四级考试，目标分数550+', type: 'medium', startDate: '2026-01-01', endDate: '2026-06-15', progress: 60, color: '#409EFF' },
  { id: 2, title: '期末复习', description: '期末考试所有科目及格，平均分80+', type: 'short', startDate: '2026-06-01', endDate: '2026-06-30', progress: 30, color: '#67C23A' },
  { id: 3, title: '考研复习', description: '准备考研，目标院校985', type: 'long', startDate: '2026-03-01', endDate: '2027-12-31', progress: 10, color: '#E6A23C' }
])

const goalTemplates = ref([
  { id: 1, title: '四六级备考', description: '系统备考英语四六级，包含听力、阅读、写作训练', icon: '📚' },
  { id: 2, title: '考研复习', description: '考研全科目复习计划，包含专业课、英语、政治', icon: '🎓' },
  { id: 3, title: '教资备考', description: '教师资格证考试备考计划，包含笔试和面试', icon: '👩‍🏫' },
  { id: 4, title: '期末冲刺', description: '期末考试复习计划，针对不同科目制定复习策略', icon: '📝' },
  { id: 5, title: '减肥打卡', description: '健康减肥计划，包含饮食和运动安排', icon: '💪' },
  { id: 6, title: '实习准备', description: '实习申请和准备计划，包含简历制作和面试准备', icon: '💼' }
])

const showAddGoalDialog = ref(false)
const newGoal = ref({
  title: '',
  description: '',
  type: 'medium',
  startDate: '',
  endDate: '',
  progress: 0,
  color: '#409EFF'
})

const addGoal = () => {
  if (newGoal.value.title) {
    goals.value.push({
      id: goals.value.length + 1,
      ...newGoal.value
    })
    showAddGoalDialog.value = false
    // 重置表单
    newGoal.value = {
      title: '',
      description: '',
      type: 'medium',
      startDate: '',
      endDate: '',
      progress: 0,
      color: '#409EFF'
    }
  }
}

const updateGoalProgress = (goal, increment) => {
  goal.progress = Math.min(100, goal.progress + increment)
}

const deleteGoal = (id) => {
  goals.value = goals.value.filter(goal => goal.id !== id)
}

const useTemplate = (template) => {
  newGoal.value = {
    title: template.title,
    description: template.description,
    type: 'medium',
    startDate: new Date().toISOString().split('T')[0],
    endDate: '',
    progress: 0,
    color: '#409EFF'
  }
  showAddGoalDialog.value = true
}

const getGoalTypeText = (type) => {
  const typeMap = {
    long: '长期',
    medium: '中期',
    short: '短期'
  }
  return typeMap[type] || type
}
</script>

<style scoped>
.goals {
  padding: 20px 0;
}

h2 {
  margin-bottom: 20px;
  color: var(--primary-color);
}

.goal-actions {
  margin-bottom: 20px;
  display: flex;
  gap: 10px;
}

.goal-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 20px;
  margin-top: 20px;
}

.goal-card {
  border-radius: var(--border-radius);
  box-shadow: var(--card-shadow);
  overflow: hidden;
  background-color: white;
}

.goal-header {
  padding: 16px;
  color: white;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.goal-header h4 {
  margin: 0;
  font-size: 16px;
  font-weight: 500;
}

.goal-type {
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  background-color: rgba(255, 255, 255, 0.3);
}

.goal-body {
  padding: 16px;
}

.goal-info {
  margin-bottom: 12px;
}

.info-item {
  display: block;
  margin-bottom: 8px;
  font-size: 14px;
  color: var(--text-color-secondary);
}

.goal-description {
  margin-bottom: 16px;
  font-size: 14px;
  line-height: 1.5;
}

.goal-progress {
  margin-bottom: 16px;
}

.progress-bar {
  width: 100%;
  height: 8px;
  background-color: #f0f0f0;
  border-radius: 4px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  border-radius: 4px;
  transition: width 0.3s ease;
}

.goal-actions {
  display: flex;
  gap: 8px;
}

.btn-sm {
  padding: 4px 12px;
  font-size: 12px;
}

/* 模板样式 */
.template-list {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 20px;
  margin-top: 20px;
}

.template-card {
  display: flex;
  padding: 16px;
  background-color: white;
  border-radius: var(--border-radius);
  box-shadow: var(--card-shadow);
  transition: var(--transition);
}

.template-card:hover {
  transform: translateY(-5px);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
}

.template-icon {
  font-size: 32px;
  margin-right: 16px;
  width: 50px;
  text-align: center;
}

.template-content {
  flex: 1;
}

.template-content h4 {
  margin: 0 0 8px 0;
  font-size: 16px;
  font-weight: 500;
}

.template-content p {
  margin: 0 0 12px 0;
  font-size: 14px;
  color: var(--text-color-secondary);
  line-height: 1.5;
}

/* 对话框样式 */
.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.dialog-content {
  background-color: white;
  border-radius: var(--border-radius);
  padding: 24px;
  width: 400px;
  max-width: 90%;
}

.dialog-actions {
  margin-top: 24px;
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

@media (max-width: 768px) {
  .goal-actions {
    flex-direction: column;
  }
  
  .goal-list {
    grid-template-columns: 1fr;
  }
  
  .template-list {
    grid-template-columns: 1fr;
  }
  
  .template-card {
    flex-direction: column;
    text-align: center;
  }
  
  .template-icon {
    margin-right: 0;
    margin-bottom: 12px;
  }
}
</style>