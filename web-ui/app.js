// API Base URL
const API_BASE = '/api';

// WebSocket connection
let ws = null;

// Initialize app
document.addEventListener('DOMContentLoaded', () => {
    initTabs();
    connectWebSocket();
    loadDashboard();
    loadAccounts();
    loadTasks();
    loadContents();
});

// Tab switching
function initTabs() {
    const tabBtns = document.querySelectorAll('.tab-btn');
    tabBtns.forEach(btn => {
        btn.addEventListener('click', () => {
            const tabName = btn.dataset.tab;
            switchTab(tabName);
        });
    });
}

function switchTab(tabName) {
    // Update buttons
    document.querySelectorAll('.tab-btn').forEach(btn => {
        btn.classList.remove('active');
    });
    document.querySelector(`[data-tab="${tabName}"]`).classList.add('active');

    // Update content
    document.querySelectorAll('.tab-content').forEach(content => {
        content.classList.remove('active');
    });
    document.getElementById(tabName).classList.add('active');

    // Load data for the tab
    if (tabName === 'accounts') loadAccounts();
    if (tabName === 'tasks') loadTasks();
    if (tabName === 'contents') loadContents();
}

// WebSocket
function connectWebSocket() {
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    ws = new WebSocket(`${protocol}//${window.location.host}/ws`);

    ws.onopen = () => {
        console.log('WebSocket connected');
        updateConnectionStatus(true);
    };

    ws.onclose = () => {
        console.log('WebSocket disconnected');
        updateConnectionStatus(false);
        // Reconnect after 5 seconds
        setTimeout(connectWebSocket, 5000);
    };

    ws.onmessage = (event) => {
        const data = JSON.parse(event.data);
        handleWebSocketMessage(data);
    };
}

function updateConnectionStatus(connected) {
    const dot = document.getElementById('connection-status');
    const text = document.getElementById('connection-text');

    if (connected) {
        dot.classList.remove('offline');
        dot.classList.add('online');
        text.textContent = '在线';
    } else {
        dot.classList.remove('online');
        dot.classList.add('offline');
        text.textContent = '离线';
    }
}

function handleWebSocketMessage(data) {
    console.log('WebSocket message:', data);

    switch (data.type) {
        case 'task_update':
            addLog('info', `任务更新: ${data.task_id} - ${data.status}`);
            loadTasks();
            break;
        case 'log':
            addLog(data.level, data.message);
            break;
        case 'stats_update':
            updateStats(data);
            break;
    }
}

// Dashboard
async function loadDashboard() {
    try {
        const [stats, status] = await Promise.all([
            fetch(`${API_BASE}/stats`).then(r => r.json()),
            fetch(`${API_BASE}/status`).then(r => r.json())
        ]);

        // Update stats
        document.getElementById('total-accounts').textContent = stats.accounts.total;
        document.getElementById('active-accounts').textContent = stats.accounts.active;
        document.getElementById('suspended-accounts').textContent = stats.accounts.suspended;

        document.getElementById('total-tasks').textContent = stats.tasks.total;
        document.getElementById('pending-tasks').textContent = stats.tasks.pending;
        document.getElementById('running-tasks').textContent = stats.tasks.running;

        document.getElementById('total-contents').textContent = stats.contents.total;

        document.getElementById('scheduler-status').textContent =
            status.scheduler_running ? '运行中' : '已停止';
    } catch (error) {
        console.error('Failed to load dashboard:', error);
    }
}

function updateStats(data) {
    document.getElementById('pending-tasks').textContent = data.pending_tasks;
    document.getElementById('running-tasks').textContent = data.running_tasks;
    document.getElementById('active-accounts').textContent = data.active_accounts;
}

function addLog(level, message) {
    const logsContainer = document.getElementById('logs');
    const emptyState = logsContainer.querySelector('.empty-state');
    if (emptyState) emptyState.remove();

    const logEntry = document.createElement('div');
    logEntry.className = `log-entry ${level}`;
    logEntry.innerHTML = `
        <div><strong>${new Date().toLocaleTimeString()}</strong> - ${message}</div>
    `;

    logsContainer.insertBefore(logEntry, logsContainer.firstChild);

    // Keep only last 50 logs
    while (logsContainer.children.length > 50) {
        logsContainer.removeChild(logsContainer.lastChild);
    }
}

// Accounts
async function loadAccounts() {
    try {
        const accounts = await fetch(`${API_BASE}/accounts`).then(r => r.json());
        const tbody = document.getElementById('accounts-table');

        if (accounts.length === 0) {
            tbody.innerHTML = '<tr><td colspan="6" class="empty-state">暂无账号</td></tr>';
            return;
        }

        tbody.innerHTML = accounts.map(account => `
            <tr>
                <td>${account.platform}</td>
                <td>${account.username}</td>
                <td><span class="badge ${account.status}">${account.status}</span></td>
                <td>${account.health_score}</td>
                <td>${account.last_active || '-'}</td>
                <td>
                    <button class="btn btn-danger" onclick="deleteAccount('${account.id}')">删除</button>
                </td>
            </tr>
        `).join('');
    } catch (error) {
        console.error('Failed to load accounts:', error);
    }
}

function showAddAccountModal() {
    document.getElementById('modal-overlay').classList.add('active');
    document.getElementById('add-account-modal').classList.add('active');
}

async function addAccount(event) {
    event.preventDefault();

    const data = {
        platform: document.getElementById('account-platform').value,
        username: document.getElementById('account-username').value,
        proxy: document.getElementById('account-proxy').value || null
    };

    try {
        await fetch(`${API_BASE}/accounts`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify(data)
        });

        closeModal();
        loadAccounts();
        loadDashboard();
        addLog('info', `添加账号: ${data.username}`);
    } catch (error) {
        console.error('Failed to add account:', error);
        alert('添加账号失败');
    }
}

async function deleteAccount(id) {
    if (!confirm('确定要删除这个账号吗？')) return;

    try {
        await fetch(`${API_BASE}/accounts/${id}`, { method: 'DELETE' });
        loadAccounts();
        loadDashboard();
        addLog('info', `删除账号: ${id}`);
    } catch (error) {
        console.error('Failed to delete account:', error);
        alert('删除账号失败');
    }
}

// Tasks
async function loadTasks() {
    try {
        const tasks = await fetch(`${API_BASE}/tasks`).then(r => r.json());
        const tbody = document.getElementById('tasks-table');

        if (tasks.length === 0) {
            tbody.innerHTML = '<tr><td colspan="6" class="empty-state">暂无任务</td></tr>';
            return;
        }

        tbody.innerHTML = tasks.map(task => `
            <tr>
                <td>${task.task_type}</td>
                <td>${task.account_id.substring(0, 8)}...</td>
                <td><span class="badge ${task.status}">${task.status}</span></td>
                <td>${task.scheduled_at || '-'}</td>
                <td>${task.executed_at || '-'}</td>
                <td>
                    ${task.status === 'pending' ?
                        `<button class="btn btn-danger" onclick="cancelTask('${task.id}')">取消</button>` :
                        '-'}
                </td>
            </tr>
        `).join('');
    } catch (error) {
        console.error('Failed to load tasks:', error);
    }
}

function showAddTaskModal() {
    alert('创建任务功能开发中...');
}

async function cancelTask(id) {
    try {
        await fetch(`${API_BASE}/tasks/${id}/cancel`, { method: 'POST' });
        loadTasks();
        loadDashboard();
        addLog('info', `取消任务: ${id}`);
    } catch (error) {
        console.error('Failed to cancel task:', error);
        alert('取消任务失败');
    }
}

// Contents
async function loadContents() {
    try {
        const contents = await fetch(`${API_BASE}/contents`).then(r => r.json());
        const tbody = document.getElementById('contents-table');

        if (contents.length === 0) {
            tbody.innerHTML = '<tr><td colspan="6" class="empty-state">暂无内容</td></tr>';
            return;
        }

        tbody.innerHTML = contents.map(content => `
            <tr>
                <td>${content.content_type}</td>
                <td>${content.platform}</td>
                <td>${content.title || content.body.substring(0, 30) + '...'}</td>
                <td>${content.used_count}</td>
                <td>${new Date(content.created_at).toLocaleDateString()}</td>
                <td>
                    <button class="btn btn-danger" onclick="deleteContent('${content.id}')">删除</button>
                </td>
            </tr>
        `).join('');
    } catch (error) {
        console.error('Failed to load contents:', error);
    }
}

function showAddContentModal() {
    alert('添加内容功能开发中...');
}

async function deleteContent(id) {
    if (!confirm('确定要删除这个内容吗？')) return;

    try {
        await fetch(`${API_BASE}/contents/${id}`, { method: 'DELETE' });
        loadContents();
        loadDashboard();
        addLog('info', `删除内容: ${id}`);
    } catch (error) {
        console.error('Failed to delete content:', error);
        alert('删除内容失败');
    }
}

// Modal
function closeModal() {
    document.getElementById('modal-overlay').classList.remove('active');
    document.querySelectorAll('.modal').forEach(modal => {
        modal.classList.remove('active');
    });
}

// Auto refresh dashboard every 10 seconds
setInterval(loadDashboard, 10000);
