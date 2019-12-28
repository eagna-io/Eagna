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

INSERT INTO organizers (id, name, thumbnail_url)
  VALUES ('ec2966c5-d661-4a9b-b377-9e00f21d7dd4', 'Eagna', 'https://eagna.io');

INSERT INTO prizes (id, name, description, thumbnail_url, point)
  VALUES ('08c70349-2066-4e47-8841-ad5bdf49c90e', 'Starbucks Coffee フードチケット（300円分）', 'Starbucks Coffee フードチケット（300円分）', 'https://s3-ap-northeast-1.amazonaws.com/giftee-photos-tokyo-biz-2013/item_sku_photos/photos/3226/w_400px/sbjfood_01_260.jpg', 300);
