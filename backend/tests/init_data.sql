INSERT INTO users (id, name, email, is_admin, credential, salt)
  VALUES (
    'f9be6c8b-2fcc-4e94-8682-7158ca51c3b5',
    'Test Admin',
    'test-admin@eagna.io',
    True,
    -- password は hogehoge
    E'\\x44C0F5C78125C8A3A6858EE2624C9EE29B9F1B9A14D845955AE5D55B00BC53FEAB9DCD6ED2C9E2CA29E83D79DAFF0A990D60BD17FC969612CDD898C3557019B1',
    E'\\x47B763B1F0794654F7AA071D9D18126A32CB66EA0E4582BABCB144368CFEA39905E1626219087E23E440E0C7FC42CA31CF5DD55EDD7045BABB377AC530522647'
  );
