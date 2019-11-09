function on_event(t, k, v) {
  window.rustimate.client.on_event(t, k, v);
};

window.rustimate = {
  on_event: on_event
};
