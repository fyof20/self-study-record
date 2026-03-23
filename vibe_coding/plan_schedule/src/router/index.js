import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    redirect: '/dashboard'
  },
  {
    path: '/dashboard',
    name: 'Dashboard',
    component: () => import('../views/Dashboard.vue'),
    meta: { title: '首页' }
  },
  {
    path: '/courses',
    name: 'Courses',
    component: () => import('../views/Courses.vue'),
    meta: { title: '课程管理' }
  },
  {
    path: '/tasks',
    name: 'Tasks',
    component: () => import('../views/Tasks.vue'),
    meta: { title: '任务规划' }
  },
  {
    path: '/goals',
    name: 'Goals',
    component: () => import('../views/Goals.vue'),
    meta: { title: '目标管理' }
  },
  {
    path: '/focus',
    name: 'Focus',
    component: () => import('../views/Focus.vue'),
    meta: { title: '专注模式' }
  },
  {
    path: '/analytics',
    name: 'Analytics',
    component: () => import('../views/Analytics.vue'),
    meta: { title: '数据洞察' }
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

// 设置页面标题
router.beforeEach((to, from, next) => {
  document.title = to.meta.title ? `${to.meta.title} - 大学生时间规划` : '大学生时间规划'
  next()
})

export default router