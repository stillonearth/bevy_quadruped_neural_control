import requests
import json
import torch
import numpy as np
from stable_baselines3 import SAC

API_STEP  = 'http://127.0.0.1:7878/step'
API_STATE = 'http://127.0.0.1:7878/state'

def send_action(action):    
    payload = json.dumps([{"action": json.dumps(action)}], indent=4)
    requests.get(API_STEP, params={'payload': payload})

def get_obs():    
    state = requests.get(API_STATE).json()
    qpos = np.array(state['qpos']).flat.copy()
    qvel = np.array(state['qvel']).flat.copy()
    cfrc_ext = np.array(state['cfrc_ext']).flat.copy()

    return np.concatenate([qpos[2:], qvel, cfrc_ext])

model = SAC.load('./sac')

while True:
    obs = get_obs()
    action = model.predict(obs)[0].tolist()
    send_action(action)

