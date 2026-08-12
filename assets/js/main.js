/* ==========================================================================
   1. التحكم في النافذة المنبثقة (Modal)
   ========================================================================== */
function openBrandModal() {
  const modal = document.getElementById('brand-modal');
  if (modal) {
    modal.classList.add('active');
  }
}

function closeBrandModal(e) {
  if (e && e.preventDefault) {
    e.preventDefault();
  }
  const modal = document.getElementById('brand-modal');
  if (modal) {
    modal.classList.remove('active');
  }

  // إذا كنا في صفحة تعديل (/web/brands/edit/id)، ارجع للرابط الرئيسي دون إعادة تحميل
  if (window.location.pathname.includes('/edit/')) {
    window.history.replaceState({}, document.title, '/web/brands');
  }
}

/* ==========================================================================
   2. إغلاق الـ Modal وقائمة البحث عند النقر خارجها
   ========================================================================== */
window.addEventListener('click', function (e) {
  const modal = document.getElementById('brand-modal');
  if (e.target === modal) {
    closeBrandModal(e);
  }

  // إغلاق قائمة نتائج البحث المنسدلة عند النقر خارجها
  const searchContainer = document.querySelector('.search-container');
  const searchResults = document.getElementById('search-results');
  if (searchContainer && searchResults && !searchContainer.contains(e.target)) {
    searchResults.innerHTML = '';
  }
});

/* ==========================================================================
   3. نظام التنبيهات (Toasts / Alerts)
   ========================================================================== */
function dismissToast(btn) {
  const toast = btn.closest('[data-toast]') || btn.closest('.toast');
  if (toast) {
    dismissToastElement(toast);
  }
}

function dismissToastElement(toastEl) {
  if (!toastEl) return;
  toastEl.classList.add('toast-hiding', 'fade-out');
  setTimeout(() => {
    toastEl.remove();
  }, 400);
}

// تشغيل عند اكتمال تحميل الصفحة
document.addEventListener('DOMContentLoaded', () => {
  // أ) تنظيف الـ URL من ?ok= لتجنب تكرار التنبيه عند عمل Refresh
  if (window.location.search.includes('ok=')) {
    const cleanUrl = window.location.pathname;
    window.history.replaceState({}, document.title, cleanUrl);
  }

  // ب) تفعيل التوقيت التلقائي (5 ثوان) وشريط التقدم لجميع التنبيهات الموجودة
  const toasts = document.querySelectorAll('[data-toast], .toast, #flash-alert');
  
  toasts.forEach((toast) => {
    // تشغيل أنيميشن شريط التقدم إن وجد
    const barFill = toast.querySelector('.toast-bar-fill');
    if (barFill) {
      barFill.style.transition = 'width 5s linear';
      setTimeout(() => {
        barFill.style.width = '0%';
      }, 50);
    }

    // إخفاء التوست تلقائياً بعد 5 ثوانٍ
    setTimeout(() => {
      dismissToastElement(toast);
    }, 5000);
  });
});