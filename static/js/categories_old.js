    function dismissToast(btn) {
      const toast = btn.closest('[data-toast]');
      if (!toast) return;
      toast.classList.add('toast-hide');
      setTimeout(() => toast.remove(), 200);
    }

    document.querySelectorAll('[data-toast]').forEach((toast) => {
      setTimeout(() => {
        toast.classList.add('toast-hide');
        setTimeout(() => toast.remove(), 200);
      }, 5000);
    });

    // مسح ?edit=... / ?ok=... من شريط العنوان بعد عرض الرسالة، عشان لو المستخدم
    // عمل Refresh بعد كده، السيرفر ميرجعش نفس رسالة النجاح تاني.
    if (window.location.search) {
      const cleanUrl = window.location.pathname;
      window.history.replaceState({}, document.title, cleanUrl);
    }
