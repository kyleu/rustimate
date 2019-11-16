function on_event(t, k, v) {
  window.rustimate.client.on_event(t, k, v);
};

function notify(level, content) {
  UIkit.notification(content, { status: level });
};

window.rustimate = {
  on_event: on_event,
  notify: notify
};
